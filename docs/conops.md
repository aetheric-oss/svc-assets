![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# Concept of Operations - `svc-assets`

Note on delegation: only allows for one level delegation. Only allows for asset
group delegations. If user wants to delegate a single asset, a group
must be created for this asset.

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       | [@GoodluckH](https://github.com/GoodluckH)                        |
| Status        | Draft                                                             |

## :telescope: Overview

The `svc-assets` micro-service provides a comprehensive asset management
solution for operators. It enables clients to perform various actions
such as registering, updating, and grouping assets. Additionally, it
facilitates the delegation of grouped assets from one operator to
another. This service is accessible to existing systems through a REST
API interface.

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md) | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-assets`](https://nocodb.arrowair.com/dashboard/#/nc/view/08f51c89-565d-40b4-984e-9ed75eea1f26) | Requirements and user stories for this microservice.
[Interface Control Document (ICD) - `svc-assets`](./icd.md) | Defines the inputs and outputs of this microservice.
[Software Design Document (SDD) - `svc-assets`](./sdd.md) | Specifies the internal activity of this microservice.

## :raised_hands: Motivation

Managing assets for operators can be a complex and challenging task.
Traditional methods of asset management are often cumbersome and
inefficient, making it difficult for operators to keep track of their
assets and collaborate with other operators to maximize their assets'
earnings potential.

The `svc-assets` micro-service was developed to address this problem. It provides a comprehensive solution that simplifies the process of registering, updating, and grouping assets. Additionally, it enables operators to delegate grouped assets to other operators, improving collaboration and efficiency.

One of the key features of the `svc-assets` micro-service is its seamless integration into Arrow's platform software. This integration provides a uniform and smooth experience for operators and allows them to easily manage their assets within the platform, reducing the complexity of managing assets across multiple systems.

Furthermore, the `svc-assets` micro-service also helps Arrow's backend services to effectively dispatch and schedule flights by providing real-time visibility into asset availability, this allows operators to quickly identify and allocate available assets to flights, improving scheduling efficiency and reducing delays.

In summary, the `svc-assets` micro-service addresses the challenges of managing assets for operators by providing a comprehensive solution that simplifies the process, improves collaboration and enhances operational efficiency.

## Needs, Goals and Objectives of Envisioned System
The needs, goals and objectives of the envisioned system revolve around addressing the challenges of managing assets for operators, simplifying the process and enhancing operational efficiency.

`svc-assets` addresses the need for a comprehensive solution that allows
operators to easily register, update and group their assets, as well as
delegate grouped assets to other operators. It aims to streamline the
asset management process and improve the efficiency of Arrow's
flight scheduling service by
providing real-time visibility into asset availability, enabling
Arrow's backend services to quickly identify and allocate available assets to flights.

Additionally, the system is designed to seamlessly integrate into Arrow's platform software, providing a unified experience for operators, and improve the backend services' efficiency.

The objectives of the system are to:

- Offer a comprehensive solution for asset management
- Facilitate the delegation of grouped assets between operators
- Integrate into Arrow's platform software
- Provide real-time visibility into asset availability
- Simplify the process of managing assets for operators and improve the
  efficiency of the flight scheduling capability

Overall, the envisioned system is aimed to address the current challenges of managing assets for operators by providing an integrated, easy-to-use solution that streamlines the process and enhances operational efficiency.
## Overview of System and Key Elements

## External Interfaces
See the ICD for this microservice.

## Proposed Capabilities
The `svc-assets` micro-service will offer a range of features to operators, which are designed to simplify the process of managing assets and enhance operational efficiency. These capabilities include:

- **Asset registration**: The service will allow operators to easily register new assets and update existing asset information.

- **Asset grouping**: The service will enable operators to group assets together, making it easier to manage and delegate them.

- **Asset delegation**: The service will allow operators to delegate grouped assets to other operators, improving collaboration and efficiency.

- **Real-time asset visibility**: The service will provide real-time
  visibility into asset availability, allowing operators to quickly
  identify and allocate available assets to flights.

- **Integration with Arrow's platform software**: The service will
  interact with other Arrow's micro-services through gPRC interfaces to
  provide a uniform and smooth experience for operators and to enable
  efficient and accurate flight scheduling.

- **REST API Interface**: The service will be accessible through a REST
  API interface, making it easy to integrate into existing systems.

 - **Reporting and Analytics**: The service will provide reporting and analytics capabilities for operators to have a better understanding of assets usage and performance

Overall, the proposed capabilities of the `svc-assets` micro-service are aimed at simplifying the process of managing assets for operators and enhancing operational efficiency.

## Modes of Operation
The `svc-assets` micro-service will have two modes of operation:

1. **Interactive mode**: This mode will allow operators to interact with the service through a user interface. Operators will be able to register new assets, update existing asset information, group assets together, and delegate grouped assets to other operators. They will also be able to view real-time asset availability and schedule flights accordingly.

2. **Autonomous mode**: This mode will allow the service to operate independently, with little or no human intervention. This mode is useful for scenarios where the service needs to continuously monitor the availability of assets and schedule flights automatically.

Both modes of operation will be accessible through REST APIs, which makes the service easy to integrate into existing systems.

In summary, the `svc-assets` micro-service will offer two modes of operation, interactive mode and autonomous mode, that enables operators to interact with the service through a user interface and operate independently, with real-time asset availability monitoring and scheduling flights accordingly. Additionally, both modes are accessible through REST APIs which allows easy integration with existing systems.
## Operational Scenarios, Use Cases and/or Design Reference Missions
The `svc-assets` micro-service is designed to support a wide range of operational scenarios and use cases for operators of VTOL aircraft, vertiports, and vertipads, including:

- **Asset registration**: United VTOL, a VTOL operator, can use the service to register new VTOL aircraft into their fleet, providing detailed information such as make, model, and serial number, as well as attaching maintenance records, ensuring all the assets are tracked and maintained properly.

- **Asset management**: Air Cargo Ops, a VTOL operator, can use the service to track and manage their fleet of VTOL aircraft, allowing them to view real-time asset availability and schedule maintenance and repairs accordingly.

- **Asset delegation**: VertiPort Co, a company that manages a network
  of vertiports, can use the service to delegate grouped assets such as
  vertipads to different operators and set permissions for these
  operators to access and manage them, improving collaboration and
  efficiency.

- **Real-time asset tracking**: Sky Taxi Inc, a VTOL taxi company, can use the service to track their fleet of VTOL aircraft in real-time, allowing them to optimize routes and improve flight times.

- **Reporting and Analytics**: AeroNet, a VTOL network operator, can use the service to track the usage and performance of their VTOL aircraft, identifying opportunities for maintenance, repairs and fleet optimization.

In summary, the `svc-assets` micro-service is designed to support a wide range of operational scenarios and use cases for operators of VTOL aircraft, vertiports, and vertipads, providing flexibility to different types of companies to manage their assets in a way that works best for their specific needs.
## Nominal & Off-Nominal Conditions
N/A
## Physical Environment

See the High-Level CONOPS.

## Support Environment

See the High-Level CONOPS.

## Impact Considerations
N/A
## Environmental Impacts
N/A
## Organizational Impacts
The `svc-assets` micro-service may have several impacts on the organization and its operations, including:

1. **Workflow and processes**: The service might change the way operators manage their assets, which may require new processes and workflows to be developed and implemented.

2. **Training and education**: Operators may need to be trained on how to use the service, and education may be necessary to ensure that operators are able to effectively use the service.

3. **Communication and collaboration**: The service might facilitate communication and collaboration among operators, which may improve the efficiency of operations.

4. **Decision-making**: The service might provide operators with real-time visibility into asset availability and usage, which may improve decision-making and allow operators to optimize their operations.

5. **Costs**: The implementation and maintenance of the service may have an impact on costs, but the service is expected to provide a return on investment by improving efficiency and reducing costs in the long run.

6. **Culture change**: The service may require operators to adapt to a new way of working, which may be a cultural change for some operators.

To address these potential impacts, it's recommended to:
- Develop a plan for the implementation of the service, including the development of new processes and workflows.
- Develop a comprehensive training program for operators
- Communicate the benefits and changes that the service will bring to the organization and its operations, to help with the culture change.
- Consider the potential costs of the service and develop a budget accordingly.
- Monitor the service's performance and make necessary adjustments to ensure its success in the long run.
- Continuously educate and train the operators to ensure they are utilizing the full potential of the service.
- Regularly review the service's performance and effectiveness to ensure it is meeting the organization's needs.

In summary, the` svc-assets` micro-service may bring significant changes
to customer organizations workflow and processes, it's important to
develop a plan for its implementation and to educate and train the
operators. The service may also bring costs and require a culture
change, it's important to consider these factors and help operators
develop a budget accordingly.

The service should be regularly monitored to ensure it is meeting the
organization's needs and making necessary adjustments. This will help
organizations to fully realize the benefits of the service and optimize
their operations.
## Technical Impacts
 The `svc-assets` micro-service will have several technical impacts on the infrastructure and systems it is integrated with, including:

- **Performance**: The service will have an impact on the performance of the systems it is integrated with, as it will require additional resources to handle the increased workload.

- **Scalability**: The service will need to be designed to be scalable, to handle an increasing number of assets and operators, and to ensure that it can continue to function effectively as the number of assets and operators grows.

- **Integration**: The service will need to be integrated with Arrow's platform software and other systems, and this integration may require additional resources and expertise.

- **Data storage**: The service will require a significant amount of data storage to store asset information and historical data, this will have an impact on the infrastructure.

- **Security**: The service will need to include robust security measures to protect sensitive data and ensure the integrity of the system.

- **Support**: The service will need ongoing support and maintenance to ensure that it continues to function effectively and to address any issues that may arise.


## Risks and Potential Issues
The `svc-assets` micro-service may encounter several challenges, including:

- **Server downtime**: The service relies on servers to function, and if these servers go down, the service will not be available. This could lead to delays in asset management and scheduling flights.

- **Data security**: The service handles sensitive asset information, and if proper security measures are not implemented, this data may be compromised.

- **Integration issues**: The service is designed to integrate with Arrow's platform software, but if there are compatibility issues, the integration may not work as expected.

- **Data accuracy**: The service relies on accurate and up-to-date data to function effectively, if the data is inaccurate, the service may not provide accurate real-time visibility into asset availability and scheduling flights may be affected.

- **Scalability**: As the number of assets and operators increases, the service may require additional resources to handle the increased workload. If the service is not scalable, it may become overwhelmed and unable to handle the increased load.

- **Lack of adoption**: If operators do not adopt the service, it may not be used to its full potential, and the benefits it provides may not be fully realized.

To mitigate these risks and potential issues, the service will include robust security measures to protect sensitive data, regular testing to ensure compatibility with Arrow's platform software, and regular maintenance to ensure the servers are up and running. Additionally, the service will be designed to be scalable, to handle an increasing number of assets and operators. The service will also include a comprehensive training program to ensure that operators are able to effectively use the service and realize its full potential.
## Appendix A: Citations
N/A
## Appendix B: Acronyms & Glossary
N/A
