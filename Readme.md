# Interstellar-tx
by Franklin Blanco

### What?
This is a library to use the functionality of database transactions across micro services. Hence the term, interstellar. :)

### Why?
When having many interdependent micro-services, each service can use transactions, and everything is fine. In a fantasy world, this is all that is needed.

But, in the real world, a signle page load calls many different micro-services, and what happens when you have a backend call that invokes many insertions to the database, from different microservices and something goes wrong, say in the last microservice. Well, the transactions will only work for that last microservice, all the other ones will stay persisted in the database and you'll have a big problem in the backend that will only be solved manually.

### How?
Service A (Proxy) sends a request to Service B (Any service). Service B returns whatever the request returns & a Response header indicating the ITx Uuid.
Service A stores this ITx Uuid in a Vec<(Uuid, ServiceName/Ip)>. It then sends another request in the same thread (route) to Service C & D. 
Service A Also stores the ITx Uuid for Service C, but Service D fails, sends back a 400 Bad request. Service A now knows to abort all the pending transactions.
Service A just has to loop through the Vec<(Uuid, ServiceName/Ip)> and call the abort route on each.

### Security
Assuming you are running open microservices (E.G. Not behind a proxy) (Not recommended). 
This will use key based authentication. Meaning, you will have an environment variable that is the parent's key, and you'll have that in each service.

I know this is insecure if the user does not have the right protocols, but I can only do so much effort. This is designed to work behind a proxy, as the proxy is the master. It decides when to commit a transaction and when to revert it.