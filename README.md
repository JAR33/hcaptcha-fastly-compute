# hcaptcha-fastly-compute

hCaptcha Serverless on Fastly Compute@Edge (Rust)

## Introduction

This code lets you easily create a Serverless hCaptcha WAF to stop bad requests to protected endpoints at the edge.

It runs on Fastly's Compute@Edge platform, sitting in between the client and your backend server.

To use it, simply add the hCaptcha JS to your page and make sure requests to protected endpoints do the following:

- match a path you have defined in the **protected_paths** config
- send a `X-hCaptcha-Response` header with the hCaptcha token, i.e. the response returned by `hcaptcha.execute()` success callback, in XHRs to your origin server's protected endpoints.

When a protected path is requested, this code will check for the token in the client request.

- If a token is found, it will call the hCaptcha service endpoint to validate and let the request through if it passes.
- If the token is missing, invalid, or expired, the client will receive an error and the request will never reach your backend.

## Configuration

Using Fastly Web UI create a new dictionary with the name "hcaptcha"

Add the following items:

| Key                           | Sample value                               | Required |
|-------------------------------|--------------------------------------------|----------|
|protected_paths                |`/t?st`, `/login`, `/auth/*`                | Yes      |
|sitekey                        |20000000-ffff-ffff-ffff-000000000002        | Yes      |
|secret_key                     |0x0000000000000000000000000000000000000000  | Yes      |
|method                         |POST                                        | No       |
|shared_secret                  |TheSecret                                   | No       |
|keep_hcaptcha_response_header  |0                                           | No       |

`protected_paths`: is a comma separated list of regex patterns for protected paths

`sitekey` and `secret_key` should be taken from https://www.hcaptcha.com/ (sitekey and account secret used for backend validation)

`shared_secret` (optional) is a shared security key sent to the backend via the `X-hCaptcha-Edge-Secret` header. You can check this in your backend code to validate that the request was in fact processed at the edge.

`keep_hcaptcha_response_header` (default: 0) - if set to 1 then forward X-hCaptcha-Response to the Origin. In general you can leave this off, as the response is a single-use token that has already been consumed by the edge.


## Backends

Using Fastly Web UI create two Hosts:

| Name              | Address                 | Enable TLS? |
|-------------------|-------------------------|-------------|
| hCaptcha          | hcaptcha.com            | Yes         |
| Origin            | (your Origin address)   |             |


## How to compile

Follow this guild: https://developer.fastly.com/learning/compute/


## How to deploy

Run this command:
`fastly compute deploy`


## Local Development

Adjust `local_server.backends.Origin` in `fastly.toml` file to point to your Origin,
then run this command:

`fastly compute serve`


## Testing from your Browser

This code ships using the "Enterprise Publisher Safe" [integration test keys](https://docs.hcaptcha.com/#test-key-set-enterprise-account-safe-end-user).

Since the response expected is `20000000-aaaa-bbbb-cccc-000000000002` you can easily check frontend behavior as follows:

```js
  var response = '20000000-aaaa-bbbb-cccc-000000000002';
  var xhr = new XMLHttpRequest();
  var jsondata = JSON.stringify({login: 'value'});
  xhr.open("POST", 'YOUR-fastly-hcaptcha-protected-endpoint');
  xhr.setRequestHeader('X-hCaptcha-Response', response);
  xhr.send(jsondata);
```
