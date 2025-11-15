import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as ecr from "aws-cdk-lib/aws-ecr";
export class AcInfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const acRepository = new ecr.Repository(this, "ACRepository", {
      repositoryName: "ac-repository",
    });
  }
}
