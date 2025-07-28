# Microservice Decomposition Experiments

This repository contains experiments performed on the [Food Delivery Modular Monolith](https://github.com/mehdihadeli/food-delivery-modular-monolith/) codebase.

## Repository Structure

### 1. Behavioral Diagram Generation
[behavioural-diagram-generation](./behavioural-diagram-generation/README.md)
Contains the complete process and artifacts for generating sequence diagrams using GitIngest context analysis.

### 2. Component Analysis from Sequence Diagrams
[component-from-sequence-only](./component-from-sequence-only/README.md)
An experiment to evaluate how well component boundaries can be inferred using only sequence diagrams, without access to the underlying codebase.

### 3. Full System Decomposition
[decomposition-including-behavioural](./decomposition-including-behavioural/README.md)
The primary experiment combining both sequence diagrams and static code analysis to propose an optimal microservice decomposition strategy.