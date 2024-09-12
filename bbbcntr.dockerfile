FROM bitwalker/alpine-elixir-phoenix:latest

WORKDIR /app

COPY ./bbbcntr .

# WORKDIR /app/bbbcntr

RUN ls -Rla
RUN mix deps.get --only prod
RUN MIX_ENV=prod mix compile
# RUN npm install --prefix assets
# RUN npm run deploy --prefix assets
RUN MIX_ENV=prod mix assets.deploy
RUN MIX_ENV=prod mix phx.digest
# RUN export SECRET_KEY_BASE="$(mix phx.gen.secret)"
RUN MIX_ENV=prod mix release

CMD ["_build/prod/rel/bbbcntr/bin/bbbcntr", "start"]

# FROM bitwalker/alpine-elixir-phoenix:latest

# WORKDIR /app

# COPY ./bbbcntr .

# # WORKDIR /app/bbbcntr

# RUN ls -Rla
# RUN mix deps.get
# RUN mix compile
# # RUN npm install --prefix assets
# # RUN npm run deploy --prefix assets
# RUN mix assets.deploy
# RUN mix phx.digest
# # RUN export SECRET_KEY_BASE="$(mix phx.gen.secret)"
# RUN mix release

# CMD ["_build/dev/rel/bbbcntr/bin/bbbcntr", "start"]