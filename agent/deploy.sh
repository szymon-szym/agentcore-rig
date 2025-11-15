aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 765444088049.dkr.ecr.us-east-1.amazonaws.com
docker buildx build --platform linux/arm64 -t ac-rig .
docker tag ac-rig:latest 765444088049.dkr.ecr.us-east-1.amazonaws.com/ac-rig:latest
docker push 765444088049.dkr.ecr.us-east-1.amazonaws.com/ac-rig:latest
