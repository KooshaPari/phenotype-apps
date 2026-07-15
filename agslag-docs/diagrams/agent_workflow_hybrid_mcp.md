vflowchart TD
    User --> UI
    UI --> Agent
    Agent --> Cache
    Agent --> RAG
    Agent --> Oblix
    Agent --> CentralMCP
    Agent --> LocalMCP

    subgraph Oblix
        EdgeModel
        CloudModel
    end

    Oblix --> EdgeModel
    Oblix --> CloudModel

    subgraph CentralMCP
        TeamComms
        SharedPlugins
    end

    subgraph LocalMCP
        PrivatePlugins
        AutoRegister
    end

    RAG --> VectorDB
    Cache -->|Hit| Agent
    Cache -->|Miss| Oblix