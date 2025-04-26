
1. [Sec3](#sec3)
   1. [Project Planning](#project-planning)

# Sec3 

1. This section is focused in simulating a Bank DB. Using structs and methods in rust. 
2. This document will also haved a dump of notes


```mermaid 
flowchart LR
    Bank[Bank] --> Account1[Account<br>balance: Integer<br>id: Unsigned Integer<br>holder: String]
    Bank --> Account2[Account<br>balance: Integer<br>id: Unsigned Integer<br>holder: String]
    Bank --> Account3[Account<br>balance: Integer<br>id: Unsigned Integer<br>holder: String]
```

## Project Planning 

1. Accounts will be sructs with the following fields:
   - balance: Integer
   - id: Unsigned Integer
   - holder: String