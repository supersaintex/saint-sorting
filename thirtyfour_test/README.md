# thirtyfour test

test selenium rust interface API

1. install selenium server from official [URL](https://www.selenium.dev/downloads/)
2. install chromedriver corresponding to your chrome version from official [URL](https://chromedriver.chromium.org/downloads)
3. copy each driver to /saint-sorting/thirtyfour-test/
	
like this
	thirtyfour_test
	├── Cargo.lock
	├── Cargo.toml
	├── chromedriver
	├── README.md
	├── selenium-server-4.8.1.jar
	├── src
	└── target

## How to run (step by step)
1. run selenium

```
java -jar selenium-server-4.8.1.jar --ext example.jar:dir standalone
```

2. run our app in /saint-sorting/ `cargo run`
3. run this test in /saint-sorting/thirtyfour_test `cargo run`

## How to run (1 step)
run script in /saint-sorting/
 ``./run_with_test.zsh
