# Actors:

1. http channel: talks to http server provider and accepts POST request ( GET is for debug and will be removed)
to deploy: add provider http-server ( from registry wasmcloud.azurecr.io/httpserver:0.16.2)  with contract ID wasmcloud:httpserver. Add link between the actor and the http-server provider. In key value specify address, e.g. address=0.0.0.0:8080

2. api gateway ( which is router): accepts post request and calls the agent reposndible for translation (chatlog - another misleading name).  Deploy and connect to a key-value provider (wasmcloud.azurecr.io/kvredis:0.17.0), the link should contain the redis URL, e.g. URL=redis://127.0.0.1:6379 as a key-value pair. 

3. chatlog, agent for translation. To deploy, also deploy providers: http-client (wasmcloud.azurecr.io/httpclient:0.6.0) with contract ID 	wasmcloud:httpclient and key value provider( wasmcloud.azurecr.io/kvredis:0.17.0) with contract ID wasmcloud:keyvalue. Add correponding links. The links with redis (key value provider) should contain the following key value URL=redis://127.0.0.1:6379
4. cohere actor for prettify. Requires links to http-client and key-value provider just like chatlog actor, including the redis URL.
5. chatgpt actor for prettify. Requires links to http-client and key-value provider just like chatlog actor, including the redis URL.
6. insert key-value pairs into the redis database:
   1. `CHATLOG_KEY "{your microsoft translate key}"`
   2. `COHERE_KEY "Bearer {your cohere api key}"`
   3. `CHATGPT_KEY "Bearer {your openai api key}"`
   4. `translate "mcchat/chatlog"`
   5. `prettify "mcchat/chatgpt"` (for chatgpt)
   6. `prettify "mcchat/cohere"` (for cohere)

Sending requests: curl -X POST http://localhost:8080/messages -d '{"body":{"user_name":"tester","body":"sheep goes somehwere"},"method":"translate"}'
or use the google-chrome extension with your URL

Changing between cohere and chatgpt for the prettify service can be done any time through changing the value  to `pretify` key as show above