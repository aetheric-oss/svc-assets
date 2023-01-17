# Software Design Document (SDD) - `svc-assets` 

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

This document details the software implementation of `svc-assets`.

This service is responsible for managing operators' assets.

Attribute | Description
--- | ---
Status | Draft
Stuckee | [@GoodluckH](https://github.com/GoodluckH)

## Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md) | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-assets`](https://nocodb.arrowair.com/dashboard/#/nc/view/08f51c89-565d-40b4-984e-9ed75eea1f26) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-assets`](./conops.md) | Defines the motivation and duties of this microservice.
[Interface Control Document (ICD) - `svc-assets`](./icd.md) | Defines the inputs and outputs of this microservice.

## Module Attributes

Attribute | Applies | Explanation
--- | --- | ---
Safety Critical | No | The module does not have direct impact on human safety.
Realtime | Yes | The module tracks the real-time availability of assets.

## Logic 

### Initialization

At initialization this service creates two servers on separate threads:
a GRPC server and a REST server. 

:exclamation: The GRPC server exists only for health-checking purposes
(i.e. Is the server up and running?).

The REST server expects the following environment variables to be set:
- `DOCKER_PORT_REST` (default: `8000`)

The GRPC server expects the following environment variables to be set:
- `DOCKER_PORT_GRPC` (default: `50051`)
### Control Loop

As a REST and GRPC server, this service awaits requests and executes handlers.

Some handlers **require** the following environment variables to be set:
- `STORAGE_HOST_GRPC`
- `STORAGE_PORT_GRPC`

This information allows `svc-assets` to connect to other microservices to obtain information requested by the client.

:exclamation: These environment variables will *not* default to anything if not found. In this case, requests involving the handler will result in a `503 SERVICE UNAVAILABLE`.

For detailed sequence diagrams regarding request handlers, see [REST Handlers](#rest-handlers).

### Cleanup

None

## REST Handlers
:construction: The `svc-assets` current handles REST requests locally.
However, these requests will require interaction with `svc-storage`.
This section will abe completed once the server finishes `svc-storage`
integration. 
