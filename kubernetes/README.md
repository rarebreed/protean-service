# kubernetes configuration

This folder contains files that will setup the entire service for us.  We will have various k8s config files for:

- The warp backend server
  - 2 nodes
- Nginx load balancer
- ElasticSearch database
  - 2 nodes, 1 replica shard each


## Responsibilities

Each of the services will run in their respective pods and will perform the following duties:

- khadga-deploy: backend server written in warp for chatting, to serve up the protean SPA, and as webrtc relay
  - Uses 2 nodes for availability
  - Deployment type
- nginx: load balancer for khadga
  - Deployment type
  - Ingress object
- elasticsearch: database storage for documents/data
  - 2 nodes with 1 replica shard
- hdfs: storage blob (only for testing.  Use cloud storage for production)
  - 2 nodes

