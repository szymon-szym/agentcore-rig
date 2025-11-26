import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as cognito from "aws-cdk-lib/aws-cognito";

export class AcInfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const acRepository = new ecr.Repository(this, "RustAgentRepository", {
      repositoryName: "agentcore-rust-agent-repo",
    });

    const userPool = new cognito.UserPool(this, "UserPool", {
      userPoolName: "AgentCoreUserPool",
      selfSignUpEnabled: true,
      signInAliases: {
        email: true,
      },
    });

    const userPoolClient = new cognito.UserPoolClient(this, "UserPoolClient", {
      userPool,
      generateSecret: true,
      authFlows: {
        adminUserPassword: true,
      },
      oAuth: {
        flows: {
          authorizationCodeGrant: true,
        },
        scopes: [cognito.OAuthScope.OPENID],
      },
    });

    const discoveryUrl = `https://cognito-idp.${this.region}.amazonaws.com/${userPool.userPoolId}/.well-known/openid-configuration`;

    new cdk.CfnOutput(this, "CognitoDiscoveryUrl", {
      value: discoveryUrl,
    });

    new cdk.CfnOutput(this, "CognitoClientId", {
      value: userPoolClient.userPoolClientId,
    });

    new cdk.CfnOutput(this, "ECRRepositoryURI", {
      value: acRepository.registryUri,
    });

    new cdk.CfnOutput(this, "ECRRepositoryName", {
      value: acRepository.repositoryName,
    });
  }
}
