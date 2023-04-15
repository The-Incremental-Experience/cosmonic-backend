# Actors:

1. http channel: talks to http server provider and accepts POST request ( GET is for debug and will be removed)
to deploy: add provider http-server with contract ID wasmcloud:httpserver. Add link between the actor and the http-server provider. In key value specify address, e.g. address=0.0.0.0:8080

1. api gateway ( which is router): accepts post request and calls the agent reposndible for translation (chatlog - another misleading name).  Deploy directly. No providers and no links are here. I think later makes sense to add some database provider and it will read something from there before calling the right agent for translation/ imporiving the text.

3. chatlog, agent for translation. To deploy, also deploy providers: http-client with contract ID 	wasmcloud:httpclient. and key value provider with contract ID wasmcloud:keyvalue. Add correponding links. The links with redis (key value provider) should I contain  the following key value URL=redis://127.0.0.1:6379
