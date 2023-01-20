# Interface Control Document (ICD) - `svc-assets`

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

This document defines the gRPC and REST interfaces unique to the `svc-assets` microservice.

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
[Software Design Document (SDD) - `svc-assets`](./sdd.md) | Specifies the internal activity of this microservice.

## Frameworks

See the High-Level ICD.

## REST

See the High-Level ICD for common interfaces.


### Files

| File Location | Description |
--- | ---
[`server/src/rest_api.rs`](../server/src/rest_api.rs) | Implements the REST endpoints.

### Authentication

See the High-Level ICD.

### Endpoints

See [TODO](./) for REST endpoints documentation.

## gRPC

### Files

These interfaces are defined in a protocol buffer file,
[`proto/svc-assets-grpc.proto`](../proto/svc-assets-grpc.proto).

### Integrated Authentication & Encryption

See the High-Level ICD.

### gRPC Server Methods ("Services")

| Service | Description |
| ---- | ---- |
| `IsReady` | Returns a message indicating if this service is ready for requests. <br>Similar to a health check, if a server is not "ready" it could be considered dead by the client making the request.
