AwsIoTDeviceEnpoint=ssl://YOUR_ENDPOINT:443
AwsIoTDeviceName=YOUT_DEVICE
AwsRootCAPath=YOUR_PATH.pem 
AwsThingCertPath=YOUR_PATH.pem
AwsThingPrivateKeyPath=YOUR_PATH.key
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
	go run github.com/tointernet/aws.iot.core.and.paho.client/golang