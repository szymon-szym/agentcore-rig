import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as iam from "aws-cdk-lib/aws-iam";
import { aws_bedrockagentcore as agentcore } from "aws-cdk-lib";

const AGENT_NAME = "rig_agent";

export class AcInfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const acRepository = new ecr.Repository(this, "ACRepository", {
      repositoryName: "ac-repository",
    });

    const runtimeRole = new iam.Role(this, "AgentCoreRustAgent", {
      assumedBy: new iam.ServicePrincipal("bedrock-agentcore.amazonaws.com", {
        conditions: {
          StringEquals: {
            "aws:SourceAccount": this.account,
          },
          ArnLike: {
            "aws:SourceArn": `arn:aws:bedrock-agentcore:${this.region}:${this.account}:*`,
          },
        },
      }),
    });

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "ECRImageAccess",
        effect: iam.Effect.ALLOW,
        actions: ["ecr:BatchGetImage", "ecr:GetDownloadUrlForLayer"],
        resources: [
          `arn:aws:ecr:${this.region}:${this.account}:repository/${acRepository.repositoryName}`,
        ],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["logs:DescribeLogStreams", "logs:CreateLogGroup"],
        resources: [
          `arn:aws:logs:${this.region}:${this.account}:log-group:/aws/bedrock-agentcore/runtimes/*`,
        ],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["logs:DescribeLogGroups"],
        resources: [`arn:aws:logs:${this.region}:${this.account}:log-group:*`],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["logs:CreateLogStream", "logs:PutLogEvents"],
        resources: [
          `arn:aws:logs:${this.region}:${this.account}:log-group:/aws/bedrock-agentcore/runtimes/*:log-stream:*`,
        ],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "ECRTokenAccess",
        effect: iam.Effect.ALLOW,
        actions: ["ecr:GetAuthorizationToken"],
        resources: ["*"],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "xray:PutTraceSegments",
          "xray:PutTelemetryRecords",
          "xray:GetSamplingRules",
          "xray:GetSamplingTargets",
        ],
        resources: ["*"],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["cloudwatch:PutMetricData"],
        resources: ["*"],
        conditions: {
          StringEquals: { "cloudwatch:namespace": "bedrock-agentcore" },
        },
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "GetAgentAccessToken",
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock-agentcore:GetWorkloadAccessToken",
          "bedrock-agentcore:GetWorkloadAccessTokenForJWT",
          "bedrock-agentcore:GetWorkloadAccessTokenForUserId",
        ],
        resources: [
          `arn:aws:bedrock-agentcore:${this.region}:${this.account}:workload-identity-directory/default`,
          `arn:aws:bedrock-agentcore:${this.region}:${this.account}:workload-identity-directory/default/workload-identity/${AGENT_NAME}-*`,
        ],
      }),
    );

    runtimeRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "BedrockModelInvocation",
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock:InvokeModel",
          "bedrock:InvokeModelWithResponseStream",
        ],
        resources: [
          "arn:aws:bedrock:*::foundation-model/*",
          `arn:aws:bedrock:${this.region}:${this.account}:*`,
        ],
      }),
    );

    const agentRuntime = new agentcore.CfnRuntime(this, "RustAgent", {
      agentRuntimeArtifact: {
        containerConfiguration: {
          containerUri: `${acRepository.registryUri}/${acRepository.repositoryName}:latest`,
        },
      },
      agentRuntimeName: AGENT_NAME,
      networkConfiguration: {
        networkMode: "PUBLIC",
      },
      roleArn: runtimeRole.roleArn,
    });

    new cdk.CfnOutput(this, "ECRRepositoryURI", {
      value: acRepository.registryUri,
    });

    new cdk.CfnOutput(this, "ECRRepositoryName", {
      value: acRepository.repositoryName,
    });

    new cdk.CfnOutput(this, "RustAgentId", {
      value: agentRuntime.attrAgentRuntimeId,
    });

    new cdk.CfnOutput(this, "AgentRuntimeRoleArn", {
      value: runtimeRole.roleArn,
    });
  }
}
