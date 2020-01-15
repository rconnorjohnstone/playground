# My Playground Repository

This repository is just for me to play around with new technologies. It took forever just to get started with wai + Docker. But now it's at least up and running.

## Startup

Planning on creating a makefile soon, but for now you can only access the "dev" environment by running:

```bash
docker-compose up --build
```

and it will run the site (which is still more or less blank) on `localhost:3000` (the pure WAI server) and `localhost:4000` (the Yesod scaffolded app). 
