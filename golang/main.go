package main

import (
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"io/ioutil"
	"log"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

func main() {
	tlsConfig, err := newTLSConfig()
	if err != nil {
		panic(err)
	}

	opts := mqtt.NewClientOptions().
		SetClientID("SomeThing").
		AddBroker(fmt.Sprintf("ssl://%s:%d", "a1omve0r7ixfps-ats.iot.us-east-1.amazonaws.com", 443)).
		SetTLSConfig(tlsConfig)

	client := mqtt.NewClient(opts)

	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(fmt.Sprintf("failed to connect broker: %v", token.Error()))
	}

	defer client.Disconnect(250)

	for {
		time.Sleep(time.Second * 5)
		println("connected")
	}
}

func newTLSConfig() (*tls.Config, error) {
	rootCA, err := ioutil.ReadFile("/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/aws-root-ca.pem")
	if err != nil {
		return nil, err
	}
	certpool := x509.NewCertPool()
	certpool.AppendCertsFromPEM(rootCA)
	cert, err := tls.LoadX509KeyPair("/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/aws-thing-cert.pem", "/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/aws-thing-private.key")
	if err != nil {
		log.Print("error to load cert", "error", err)
		return nil, err
	}

	cert.Leaf, err = x509.ParseCertificate(cert.Certificate[0])
	if err != nil {
		log.Print("error to parse cert", "error", err)
		return nil, err
	}

	return &tls.Config{
		RootCAs:            certpool,
		InsecureSkipVerify: true,
		Certificates:       []tls.Certificate{cert},
		NextProtos:         []string{"x-amzn-mqtt-ca"},
	}, nil
}
