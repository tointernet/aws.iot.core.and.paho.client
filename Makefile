AwsIoTDeviceEnpoint=ssl://a1omve0r7ixfps-ats.iot.us-east-1.amazonaws.com:443
AwsIoTDeviceName=SomeThing
AwsRootCAPath=/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/aws-root-ca.pem 
AwsThingCertPath=/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/aws-thing-cert.pem
AwsThingPrivateKeyPath=/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/aws-thing-private.key
AwsIoTTopicToPublish=test/first
AwsIoTTopicToSubiscribe=test/first

run-rust:
	AWS_IOT_DEVICE_DATA_ENDPOINT=${AwsIoTDeviceEnpoint} \
	AWS_IOT_DEVICE_NAME=${AwsIoTDeviceName} \
	AWS_ROOT_CA_PATH=${AwsRootCAPath} \
	AWS_THING_CERT_PATH=${AwsThingCertPath} \
	AWS_THING_PRIVATE_KEY_PATH=${AwsThingPrivateKeyPath} \
	AWS_IOT_TOPIC_TO_PUBLISH=${AwsIoTTopicToPublish} \
	AWS_IOT_TOPIC_TO_SUBSCRIBE=${AwsIoTTopicToSubiscribe} \
	cargo run

run-go:
	AWS_IOT_DEVICE_DATA_ENDPOINT=${AwsIoTDeviceEnpoint} \
	AWS_IOT_DEVICE_NAME=${AwsIoTDeviceName} \
	AWS_ROOT_CA_PATH=${AwsRootCAPath} \
	AWS_THING_CERT_PATH=${AwsThingCertPath} \
	AWS_THING_PRIVATE_KEY_PATH=${AwsThingPrivateKeyPath} \
	AWS_IOT_TOPIC_TO_PUBLISH=${AwsIoTTopicToPublish} \
	AWS_IOT_TOPIC_TO_SUBSCRIBE=${AwsIoTTopicToSubiscribe} \
	go run golang/main.go