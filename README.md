# hcaptcha-fastly-compute
hCaptcha Serverless on Fastly Compute@Edge (Rust)


## Configuration

Using Fastly Web UI create a new dictionary with the name "hcaptcha"
Add the following items:

| Key              | Test value |                               | Required |
|----------------------------------------------------------------------------
|protected_paths   |/t?st, /login, /auth/*                      | Yes
|sitekey           |20000000-ffff-ffff-ffff-000000000002        | Yes
|secret_key        |0x0000000000000000000000000000000000000000  | Yes
|method            |POST                                        | No

`protected_paths`: is a comma separated list of patterns for protected paths
`sitekey` and `secret_key` should be taken from https://www.hcaptcha.com/


## Backends

Using Fastly Web UI create two Hosts:

| Name              | Address                 | Enable TLS?
|----------------------------------------------------------
| hCaptcha          | hcaptcha.com            | Yes
| Origin            | (your Origin address)   |


## How to compile

Follow this guild: https://developer.fastly.com/learning/compute/


## How to deploy

Run this command:
`fastly compute deploy`


## Local Development

Adjust `local_server.backends.Origin` in `fastly.toml` file to point to your Origin,
then run this command:
`fastly compute serve`
