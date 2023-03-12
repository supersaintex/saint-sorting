# Application tests

## Required tools for selenium
* selenium server
	https://selenium-release.storage.googleapis.com/3.141/selenium-server-standalone-3.141.59.jar

* Java
	Selenium server is written in Java so you will also need Java installed.
	I confirmed that it works with the openjdk 11.0.18.

* Webdriver
	Check your version of Chrome and then download the version of chromedriver that corresponds to your version of Chrome.
	Unzip the downloaded file and place the chromedriver executable somewhere in your PATH. You can put it in the same directory as the selenium jar file if you want.

## How to run test
1. Requires chromedriver running on port xxxx:
	choromedriver --port=xxxx
	
	(You need to refer the port number from test_main.rs)

2. Run as follows:
	 cargo test

