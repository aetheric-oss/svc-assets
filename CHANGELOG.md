## [Release 0.4.1](https://github.com/Arrow-air/svc-assets/releases/tag/v0.4.1)

### 🐛 Fixes

- remove (build) from build.rs print statement ([`660bece`](https://github.com/Arrow-air/svc-assets/commit/660bece8217d0ef1e6813c28c8c1496e123a197d))

### 🔥 Refactorings

- use lib-common and new svc-storage-grpc-client ([`4349013`](https://github.com/Arrow-air/svc-assets/commit/4349013be6f115cbd32bafb61820b7b6d9cc602a))

### 🛠 Maintenance

- terraform provisioned file changes ([`17fc0b7`](https://github.com/Arrow-air/svc-assets/commit/17fc0b7d1a263d2fb318563b00b15a85ea92eef4))
- reviewer comments ([`b244482`](https://github.com/Arrow-air/svc-assets/commit/b244482f642f2f3f71d25b900fed37abb94ad302))

### 📚 Documentation

- fix icons/banners and add client READMEs ([`b42a893`](https://github.com/Arrow-air/svc-assets/commit/b42a8933613db27587a656d9a71cc6e91fa2c524))

## [Release 0.4.0](https://github.com/Arrow-air/svc-assets/releases/tag/v0.4.0)

### ✨ Features

-  **types:** add local structs for assets (#2) ([`7770fec`](https://github.com/Arrow-air/svc-assets/commit/7770fec9cf68e2cc23ec7fec057bb89a04e953a7))
-  **REST:** implement endpoints (#3) ([`1489feb`](https://github.com/Arrow-air/svc-assets/commit/1489feb2642c49132ed7ea19c414fe9aba0562eb))
- migrate assets from local struct to svc-storage ([`114e30a`](https://github.com/Arrow-air/svc-assets/commit/114e30aed39019217fa6d211314bf77767081893))
-  **demo:** add demo endpoints for getting all assets ([`16418bd`](https://github.com/Arrow-air/svc-assets/commit/16418bdedd7e2a566c6ecbfecc38790f9fb36c37))
- update svc-storage dependency ([`973eae1`](https://github.com/Arrow-air/svc-assets/commit/973eae1665be4ab5b9e5cb1f63ba7c3b5928badf))
- add health endpoint ([`30d420e`](https://github.com/Arrow-air/svc-assets/commit/30d420eaee77a795c990613beb59edd69097a4e3))

### 🐛 Fixes

-  **rest:** add tag to utoipa paths ([`44819a4`](https://github.com/Arrow-air/svc-assets/commit/44819a4995b2aaf76622c019ff85cb17432d35e5))
- add `assets` as REST path prefix ([`4bdb9ed`](https://github.com/Arrow-air/svc-assets/commit/4bdb9ed3748d00d1b9f7ea8283543ba1ed884375))
- address wrong function names in error messages ([`b511af0`](https://github.com/Arrow-air/svc-assets/commit/b511af00e27aa2614aedf545e8fccbba77c45a85))
-  **openapi:** correct OpenAPI docs ([`ac287ab`](https://github.com/Arrow-air/svc-assets/commit/ac287aba372362488310495901ad28941468f947))
- address review comments ([`4428782`](https://github.com/Arrow-air/svc-assets/commit/4428782e670f74e05fb753342bf5365399d11255))

### 🔥 Refactorings

- remove repeated grpc client impls and fix storage version ([`40b39ec`](https://github.com/Arrow-air/svc-assets/commit/40b39ec5a30bb787b483b31b8101d22a4a64d45c))
- refactor file structure ([`22c12df`](https://github.com/Arrow-air/svc-assets/commit/22c12df9f15efb012b68d31cfb2ca6a2d8347af6))
- flatten code structure ([`75a9e0e`](https://github.com/Arrow-air/svc-assets/commit/75a9e0e9ad3ffef9f27e3b44da1fda4e9f61bda8))

### 🛠 Maintenance

- terraform provisioned file changes ([`f758947`](https://github.com/Arrow-air/svc-assets/commit/f75894705e8477e1a8e7be42eb94ad38ddb4e605))
- rename template placeholders (#1) ([`7871ce6`](https://github.com/Arrow-air/svc-assets/commit/7871ce6ee3718edee026678e50c1b3dadaffdffc))
-  **cargo:** use release tags for arrow dependencies ([`acfec83`](https://github.com/Arrow-air/svc-assets/commit/acfec831ffb678e83c9dd3dc176298f96fe006e7))
- add client-rest ([`5ab7308`](https://github.com/Arrow-air/svc-assets/commit/5ab730855b2a87a9fce1b00c3da93a1343475931))
- add last_vertiport_id to assets openapi ([`bfa2b2a`](https://github.com/Arrow-air/svc-assets/commit/bfa2b2aebc4920bfcc31a639baab85ae9b794b6a))
- fix some rust-doc warnings ([`335b73b`](https://github.com/Arrow-air/svc-assets/commit/335b73b093c52155dc0c37505bd99042bd917990))
- clean up for r2 review ([`89be61e`](https://github.com/Arrow-air/svc-assets/commit/89be61efdc5237eca70db9e67e4fd418301fabf3))

### 📚 Documentation

- publish CONOPS, ICD and SDD (#4) ([`226c2da`](https://github.com/Arrow-air/svc-assets/commit/226c2da11a4874ad3797b33556826624067cfa0d))
-  **SDD:** add mermaid sequence diagram ([`b63e150`](https://github.com/Arrow-air/svc-assets/commit/b63e150e0aeb7f148dd03b71e2b011ada50a51d0))
- link endpoints to official website ([`f1555c8`](https://github.com/Arrow-air/svc-assets/commit/f1555c83bac15486e9794d51c7ef17814e8749c1))

## [Release 0.2.0-develop.2](https://github.com/Arrow-air/svc-template-rust/releases/tag/v0.2.0-develop.2)

### 🛠 Maintenance

-  **ci:** .license - provisioned by terraform ([`591d8c0`](https://github.com/Arrow-air/svc-template-rust/commit/591d8c01ba784b953077c7cf704ccd94016ee49b))
-  **ci:** .editorconfig - provisioned by terraform ([`812bf50`](https://github.com/Arrow-air/svc-template-rust/commit/812bf50e205bc73525ce5b9b0a10acc81ed032c2))
-  **ci:** .github/workflows/nightly.yml - provisioned by terraform ([`d832a55`](https://github.com/Arrow-air/svc-template-rust/commit/d832a552e45ba702d7ea452c6a3f11421bcf2e10))

## [Release 0.2.0-develop.1](https://github.com/Arrow-air/svc-template-rust/releases/tag/v0.2.0-develop.1)

### 🛠 Maintenance

-  **ci:** update changelog ([`b70bc54`](https://github.com/Arrow-air/svc-template-rust/commit/b70bc54e886924b54b06a4436c405dd885e288a9))
-  **ci:** update package version ([`3ffa96b`](https://github.com/Arrow-air/svc-template-rust/commit/3ffa96b9e219db4ca308cbf797d5225715c59218))
-  **ci:** update package version<br/><br/>[skip ci] ([`7990036`](https://github.com/Arrow-air/svc-template-rust/commit/79900366c33b4b4cf867330e2e1d672048db2025))
-  **ci:** dockerfile - provisioned by terraform ([`0fa2fdc`](https://github.com/Arrow-air/svc-template-rust/commit/0fa2fdc189664eb962a2edb6957a21cb0dcf5356))
-  **ci:** .make/rust.mk - provisioned by terraform ([`1507317`](https://github.com/Arrow-air/svc-template-rust/commit/150731785302aaed30ddee146c4fad953a9e0399))
-  **ci:** .github/workflows/release.yml - provisioned by terraform ([`eaa6150`](https://github.com/Arrow-air/svc-template-rust/commit/eaa6150bb7ff7f291c297de0333724a9f1ad9941))
-  **ci:** .commitlintrc.yml - provisioned by terraform ([`7780434`](https://github.com/Arrow-air/svc-template-rust/commit/778043483698c889969a9f6ef96176c606315abe))
-  **ci:** .make/docker.mk - provisioned by terraform ([`4a0836f`](https://github.com/Arrow-air/svc-template-rust/commit/4a0836f4a29de27a0a30c2a2ebc05fe938f4e681))
-  **ci:** .make/env.mk - provisioned by terraform ([`238b07d`](https://github.com/Arrow-air/svc-template-rust/commit/238b07dca9185162e234ca751e2b277dccd74cc9))
-  **ci:** makefile - provisioned by terraform ([`7344bbd`](https://github.com/Arrow-air/svc-template-rust/commit/7344bbd368679b3137aa06e246430069ab25a5e1))
-  **ci:** .env.base.tftpl - provisioned by terraform ([`4aa6f63`](https://github.com/Arrow-air/svc-template-rust/commit/4aa6f63b42e108cfa7c9ad15680dce7ebd3d8ed0))
-  **ci:** .env.base - provisioned by terraform ([`e50efe6`](https://github.com/Arrow-air/svc-template-rust/commit/e50efe607ba406c6fd44a322c30ec236c9473a8c))
-  **ci:** .make/env.mk - provisioned by terraform ([`6d3209c`](https://github.com/Arrow-air/svc-template-rust/commit/6d3209c141782037846c7b5e3e7bab861f19bb1d))
-  **ci:** .gitignore - provisioned by terraform ([`a370a33`](https://github.com/Arrow-air/svc-template-rust/commit/a370a336b1ac6760ca16fd1feec0d36333d87be0))
-  **ci:** .editorconfig - provisioned by terraform ([`6b23f99`](https://github.com/Arrow-air/svc-template-rust/commit/6b23f9980c989ac781cb67feb838842a50aa0377))
-  **ci:** .github/workflows/release.yml - provisioned by terraform ([`bb99bd8`](https://github.com/Arrow-air/svc-template-rust/commit/bb99bd8443ebb36de8fb606dae9974765d156a84))
-  **license:** add license and release requirements ([`245d7ec`](https://github.com/Arrow-air/svc-template-rust/commit/245d7ec8db19f78c5d73936be92f101977c546d6))
