# contrat_generator

This repository is a template for generating interfaces form centralized contracts. I use  [typeshare](https://github.com/1Password/typeshare)  of OnePassword, currently I support only Typescript. In the future, i will support kotlin, switf , go and python.

## How to start?

### Typescript part:

You can write the `contract` in the contract folder. You can find all the documentation on the definition in the  [typeshare](https://github.com/1Password/typeshare)  repo.

You must create an account on npm and generate an Access token for the CI. Save it in the GitHub secret. In the `.github/workflow`, the name of the secret is NPM_KEY.

After that, you need to set up in `ts/package.json` the package's name and description.

Moreover, usually that is all.