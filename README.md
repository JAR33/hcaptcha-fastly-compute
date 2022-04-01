# hcaptcha-fastly-compute
hCaptcha Serverless on Fastly Compute@Edge (Rust)


## Configuration

Using Fastly Web UI create a new dictionary with the name "hcaptcha"

Add the following items:

| Key                           | Sample value                               | Required |
|-------------------------------|--------------------------------------------|----------|
|protected_paths                |/t?st, /login, /auth/*                      | Yes      |
|sitekey                        |20000000-ffff-ffff-ffff-000000000002        | Yes      |
|secret_key                     |0x0000000000000000000000000000000000000000  | Yes      |
|method                         |POST                                        | No       |
|shared_secret                  |TheSecret                                   | Yes      |
|keep_hcaptcha_response_header  |0                                           | No       |

`protected_paths`: is a comma separated list of patterns for protected paths

`sitekey` and `secret_key` should be taken from https://www.hcaptcha.com/

`shared_secret` is a shared security key string that is sent to the backend. You can check this in your backend code to validate that the request was in fact processed at the edge.

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
