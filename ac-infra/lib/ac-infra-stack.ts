import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as iam from "aws-cdk-lib/aws-iam";

export class AcInfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const acRepository = new ecr.Repository(this, "ACRepository", {
      repositoryName: "ac-repository",
    });

    // user to be used by GH actions
    const ghActionsUser = new iam.User(this, "GHActionsUser", {
      userName: "gh-actions-user",
    });

    const GH_ORG = "szymon-szym";
    const GH_REPO = "agentcore-rig";
    const GH_BRANCH = "*";

    const ghActionRole = new iam.Role(this, "GitHubActionRole", {
      roleName: "gh-action-role",
      assumedBy: new iam.FederatedPrincipal(
        `arn:aws:iam::${this.account}:oidc-provider/token.actions.githubusercontent.com`,
        {
          StringEquals: {
            "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
          },
          StringLike: {
            "token.actions.githubusercontent.com:sub": `repo:${GH_ORG}/${GH_REPO}:ref:refs/heads/${GH_BRANCH}`,
          },
        },
        "sts:AssumeRoleWithWebIdentity",
      ),
      managedPolicies: [
        iam.ManagedPolicy.fromAwsManagedPolicyName("AdministratorAccess"), // or least privilege
      ],
    });

    // grant permissions to push to ECR
    acRepository.grantPush(ghActionsUser);

    new cdk.CfnOutput(this, "ECRRepository", {
      value: acRepository.registryUri,
    });

    new cdk.CfnOutput(this, "GHRoleToAssume", {
      value: ghActionRole.roleArn,
    });
  }
}
