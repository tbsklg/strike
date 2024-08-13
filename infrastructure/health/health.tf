locals {
    app_name = "strikes"
    lambda_name = "health_lambda"
}

data "aws_iam_policy_document" "lambda_assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
 }
}

resource "aws_iam_role" "lambda_role" {
    name               = "${local.app_name}-${local.lambda_name}"
    assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

resource "aws_iam_role_policy_attachment" "basic_execution_role_policy_attachment" {
    role        = aws_iam_role.lambda_role.name
    policy_arn  = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/health/bootstrap"
  output_path = "${path.module}/target/archive/health.zip"
}

resource "aws_lambda_function" "health" {
  filename      = data.archive_file.lambda_archive.output_path
  function_name = "${local.app_name}-${local.lambda_name}"
  role          = aws_iam_role.lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["arm64"]

  memory_size = 1024
}

output "lambda_invoke_arn" {
  value = aws_lambda_function.health.invoke_arn
}

output "lambda_function_name" {
  value = aws_lambda_function.health.function_name
}

