FROM scratch
# Such copy here is only for Dev-like environments. For other Envs - should be mounted explicitly.
COPY ../env /env
COPY ../target/x86_64-unknown-linux-gnu/release/service /service
CMD [ "/service" ]
