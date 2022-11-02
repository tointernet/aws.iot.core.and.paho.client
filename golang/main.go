package main

import (
	"crypto/tls"
	"crypto/x509"
	"errors"
	"io/ioutil"
	"os"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"go.uber.org/zap"
)

var (
	log, _ = zap.NewDevelopment()
	logger = log.Sugar().Named("go-aws-iot")
)

func main() {
	logger.Debug("initializing...")

	/// MQTT CLIENT AND CONNECTION
	client, err := mqttClient()
	if err != nil {
		logger.Panic(err)
	}

	if token := client.Connect(); token.Wait() && token.Error() != nil {
		logger.Panicf("failed to connect broker: %v", token.Error())
	}

	defer client.Disconnect(250)
	///

	/// MQTT PUBLISHER AND CONSUMER
	publisher(client)

	topic := ""
	if topic = os.Getenv("AWS_IOT_TOPIC_TO_SUBSCRIBE"); topic == "" {
		logger.Panic("invalid AWS_IOT_TOPIC_TO_SUBSCRIBE arg")
	}

	logger.Debug("subscribing to a topic")

	client.Subscribe("test/first", byte(1), func(c mqtt.Client, m mqtt.Message) {
		logger.Debugw("received msg", zap.String(topic, m.Topic()), "msg", m.Payload())
	})

	logger.Debug("subscribed")
	///

	forever := make(chan bool)
	<-forever
}

func mqttClient() (mqtt.Client, error) {
	tls, err := tlsConfig()

	if err != nil {
		logger.Errorw("error to create tls config", zap.Error(err))
		return nil, err
	}

	clientId, awsEndpoint := "", ""

	if clientId = os.Getenv("AWS_IOT_DEVICE_NAME"); clientId == "" {
		return nil, errors.New("invalid AWS_IOT_DEVICE_NAME arg")
	}

	if awsEndpoint = os.Getenv("AWS_IOT_DEVICE_DATA_ENDPOINT"); clientId == "" {
		return nil, errors.New("invalid AWS_IOT_DEVICE_DATA_ENDPOINT arg")
	}

	logger.Debug("connection to mqtt...")

	opts := mqtt.NewClientOptions().
		SetClientID(clientId).
		AddBroker(awsEndpoint).
		SetTLSConfig(tls)

	client := mqtt.NewClient(opts)

	logger.Debug("mqtt client connected")

	return client, nil
}

func tlsConfig() (*tls.Config, error) {
	rootCAPath, certPath, privateKeyPath := "", "", ""

	if rootCAPath = os.Getenv("AWS_ROOT_CA_PATH"); rootCAPath == "" {
		return nil, errors.New("invalid AWS_ROOT_CA_PATH arg")
	}

	if certPath = os.Getenv("AWS_THING_CERT_PATH"); certPath == "" {
		return nil, errors.New("invalid AWS_THING_CERT_PATH arg")
	}

	if privateKeyPath = os.Getenv("AWS_THING_PRIVATE_KEY_PATH"); privateKeyPath == "" {
		return nil, errors.New("invalid AWS_THING_PRIVATE_KEY_PATH arg")
	}

	rootCA, err := ioutil.ReadFile(rootCAPath)

	if err != nil {
		logger.Errorw("error to read root CA", zap.Error(err))
		return nil, err
	}

	certpool := x509.NewCertPool()
	certpool.AppendCertsFromPEM(rootCA)
	cert, err := tls.LoadX509KeyPair(certPath, privateKeyPath)

	if err != nil {
		logger.Errorw("error to load cert", zap.Error(err))
		return nil, err
	}

	cert.Leaf, err = x509.ParseCertificate(cert.Certificate[0])
	if err != nil {
		logger.Errorw("error to parse cert", zap.Error(err))
		return nil, err
	}

	return &tls.Config{
		RootCAs:            certpool,
		InsecureSkipVerify: true,
		Certificates:       []tls.Certificate{cert},
		NextProtos:         []string{"x-amzn-mqtt-ca"},
	}, nil
}

type message struct {
	Data string `json:"data"`
}

func publisher(client mqtt.Client) error {
	topic := ""
	if topic = os.Getenv("AWS_IOT_TOPIC_TO_PUBLISH"); topic == "" {
		return errors.New("invalid AWS_IOT_TOPIC_TO_PUBLISH arg")
	}

	go func() {
		for {
			time.Sleep(time.Second * 10)
			logger.Debug("publishing to mqtt")
			client.Publish(topic, byte(0), false, &message{"hello"})
			logger.Debug("published")
		}
	}()

	return nil
}
