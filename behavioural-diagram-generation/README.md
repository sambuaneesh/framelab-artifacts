## Prompts used to generate the final behavioral diagrams (Sequence Diagrams)
Note that I used these prompts to generate multiple behavioral diagrams, but we are only using the final sequence diagrams for the experiment.

The complete setup was implemented using LangChain and follows this methodology:
![alt text](./static/Pasted%20image%2020250703131537.png)

To access the diagrams, you can go to the [behavioral_diagrams.html](./index.html) file.

### Component Analysis Chains

#### 1. Module Identification

```
Based on the following system description, identify the primary software modules/components.
List them as simple phrases focusing on functional modules (e.g., 'User Management', 'Payment Processing', 'Notification Service').

Description:
{readme_content}

Identified Modules:
```

#### 2. Architectural Pattern Analysis

```
You are a solution architect analyzing architectural patterns.
Based on the following code from the '{module_name}' module, identify the high-level architectural patterns and component interactions.

Focus on architectural concerns:
- Service boundaries and interfaces
- Data flow between layers
- Integration patterns (sync/async)
- External system dependencies
- Event-driven patterns
- Cross-cutting concerns (security, logging, caching)

Avoid code implementation details. Think at the architectural abstraction level.

Code Context:
{code_context}

Architectural Patterns for {module_name}:
```

#### 3. Business Process Flow Analysis

```
You are a solution architect analyzing business process flows.
Based on the following code from the '{module_name}' module, identify the high-level business process and decision flows.

Focus on:
- Business process steps (not code steps)
- Decision points and business rules
- Integration touchpoints
- Data transformation points
- Business event flows
- Error handling strategies

Think in terms of business capabilities and process orchestration, not implementation details.

Code Context:
{code_context}

Business Process Flow for {module_name}:
```

### Diagram Generation Chains

#### 1. Architectural Sequence Diagram Generation 

```
Generate a Mermaid sequence diagram for the '{module_name}' module at the architectural level.

Pattern Analysis:
{pattern_analysis}

Process Analysis:
{process_analysis}

Create an architectural sequence diagram showing high-level interactions between:
- External actors/systems
- Business services (not code classes)
- Integration layers
- External systems/APIs
- Data stores (logical, not physical)

Focus on business capabilities and service interactions, not implementation details.
Use meaningful business/architectural names, not technical class names.

Example format:
sequenceDiagram
    participant User as "End User"
    participant Gateway as "API Gateway"
    participant AuthSvc as "Authentication Service"
    participant BizSvc as "Business Service"
    participant DataLayer as "Data Layer"
    participant ExtAPI as "External System"
    
    User->>Gateway: Business Request
    Gateway->>AuthSvc: Validate Session
    AuthSvc-->>Gateway: Session Valid
    Gateway->>BizSvc: Execute Business Logic
    BizSvc->>DataLayer: Persist Data
    BizSvc->>ExtAPI: External Integration
    BizSvc-->>Gateway: Business Response
    Gateway-->>User: Result

Mermaid Architectural Sequence Diagram:
```

#### 2. Business Process Activity Diagram Generation 

```
Generate a Mermaid flowchart showing the business process flow for the '{module_name}' module.

Process Analysis:
{process_analysis}

Pattern Analysis:
{pattern_analysis}

Create a business process flowchart showing:
- Business process steps (not code steps)
- Business decision points
- Integration points with external systems
- Error handling and exception paths
- Parallel business processes

Focus on business workflow and decision logic, avoid technical implementation details.
Use business terminology and process names.

Example format:
flowchart TD
    A[Start Process] --> B{Business Rule Check}
    B -->|Meets Criteria| C[Execute Business Logic]
    B -->|Fails Criteria| D[Handle Exception]
    C --> E{Integration Required?}
    E -->|Yes| F[Call External System]
    E -->|No| G[Complete Process]
    F --> H{External Success?}
    H -->|Success| G
    H -->|Failure| I[Compensating Action]
    I --> J[End with Error]
    G --> K[End Successfully]

Mermaid Business Process Diagram:
```

#### 3. Business Entity State Diagram Generation 

```
Generate a Mermaid state diagram for the main business entity in the '{module_name}' module.

Process Analysis:
{process_analysis}

Pattern Analysis:
{pattern_analysis}

Create a business entity state diagram showing:
- Business entity states (not code object states)
- Business events that trigger state transitions
- Business rules governing transitions
- End states and error states

Focus on the business entity lifecycle from a domain perspective.
Use business domain language, not technical terms.

Example format:
stateDiagram-v2
    [*] --> Initiated
    Initiated --> UnderReview : Submit for Review
    UnderReview --> Approved : Business Approval
    UnderReview --> Rejected : Business Rejection
    Approved --> Active : Deploy to Production
    Active --> Suspended : Business Suspension
    Suspended --> Active : Reactivate
    Active --> Archived : End of Lifecycle
    Rejected --> [*]
    Archived --> [*]

Mermaid Business Entity State Diagram:
```


#### Template Variables

|Variable|Description|Used In|
|---|---|---|
|`{readme_content}`|Content from README.md file|Module identification, System overview|
|`{module_name}`|Name of the specific module being analyzed|All module-specific prompts|
|`{code_context}`|Relevant code snippets from vector store|Pattern analysis, Process analysis|
|`{pattern_analysis}`|Output from architectural pattern analysis|Diagram generation prompts|
|`{process_analysis}`|Output from business process analysis|Diagram generation prompts|
|`{modules}`|List of all identified modules|System overview generation|

