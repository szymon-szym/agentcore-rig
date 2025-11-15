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

    // grant permissions to push to ECR
    acRepository.grantPush(ghActionsUser);
  }
}
