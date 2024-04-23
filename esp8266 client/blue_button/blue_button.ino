/*
  Rui Santos
  Complete project details at Complete project details at https://RandomNerdTutorials.com/esp8266-nodemcu-http-get-post-arduino/

  Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files.
  The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
  
  Code compatible with ESP8266 Boards Version 3.0.0 or above 
  (see in Tools > Boards > Boards Manager > ESP8266)
*/

#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <WiFiClient.h>

const char* ssid = "Appfarm";
const char* password = "ObjectInContext";

const char* urls[] = {
  "http://192.168.0.183:1337/Living%20Room/blue_button.mp3"
  // "http://192.168.0.183:1337/Revenue/blue_button.mp3"
};


// the following variables are unsigned longs because the time, measured in
// milliseconds, will quickly become a bigger number than can be stored in an int.
unsigned long lastTime = 0;
// Timer set to 10 minutes (600000)
//unsigned long timerDelay = 600000;
// Set timer to 10 seconds (10000)
unsigned long timerDelay = 10000;

// Define the button pin
const int buttonPin = D1; // Assuming D1 is connected to the button

void setup() {
  Serial.begin(115200); 

  // Initialize the button pin as input
  pinMode(buttonPin, INPUT_PULLUP);

  WiFi.begin(ssid, password);
  Serial.println("Connecting");
  while(WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  Serial.print("Connected to WiFi network with IP Address: ");
  Serial.println(WiFi.localIP());
 
  Serial.println("Timer set to 5 seconds (timerDelay variable), it will take 5 seconds before publishing the first reading.");
}

// Function to send HTTP request to a given URL and wait for response
bool sendHttpRequest(const char* url) {
 if (WiFi.status() == WL_CONNECTED) {
    WiFiClient client;
    HTTPClient http;

    http.begin(client, url);

    // Send HTTP GET request
    int httpResponseCode = http.GET();

    if (httpResponseCode > 0) {
      Serial.print("HTTP Response code: ");
      Serial.println(httpResponseCode);
      String payload = http.getString();
      Serial.println(payload);
      http.end();
      return true; // Request was successful
    } else {
      Serial.print("Error code: ");
      Serial.println(httpResponseCode);
      http.end();
      return false; // Request failed
    }
 } else {
    Serial.println("WiFi Disconnected");
    return false; // WiFi not connected
 }
}

void loop() {
 if (digitalRead(buttonPin) == LOW) {
    // Iterate over the list of URLs and send HTTP request for each
    for (int i = 0; i < sizeof(urls) / sizeof(urls[0]); i++) {
      bool requestSuccess = sendHttpRequest(urls[i]);
      if (requestSuccess) {
        Serial.println("HTTP request to " + String(urls[i]) + " completed successfully.");
      } else {
        Serial.println("HTTP request to " + String(urls[i]) + " failed.");
      }
    }
    // Optionally, add a delay here to prevent multiple requests on a single button press
    delay(500); // Adjust the delay as needed
 }
}
