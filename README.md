# AnimalDeCompagnieNoSu

[![Adoption](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/adoption.yml/badge.svg)](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/adoption.yml)
[![AnimalCenter](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/animal_center.yml/badge.svg)](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/animal_center.yml)
[![Dependabot](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/dependabot-auto.yml/badge.svg)](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/dependabot-auto.yml)
![Docker Adoption](https://img.shields.io/docker/pulls/futugyousuzu/adoption)
![Docker Adoption Migration](https://img.shields.io/docker/pulls/futugyousuzu/adoption-migration)
![Docker Animal Center](https://img.shields.io/docker/pulls/futugyousuzu/back_animal_center)
[![Markdownlint](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/markdownlint.yml/badge.svg)](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/markdownlint.yml)
[![Shuttle Deploy](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/shuttle.yml/badge.svg)](https://github.com/futugyou/AnimalDeCompagnieNoSu/actions/workflows/shuttle.yml)

## Docker

```sh
docker build -t futugyousuzu/adoption-migration:latest -f ./Adoption/Adoption.DbMigrator/Dockerfile ./Adoption
docker build -t futugyousuzu/adoption:latest -f ./Adoption/Adoption.Host/Dockerfile ./Adoption

docker build -t futugyousuzu/back_animal_center:latest -f ./AnimalCenter/Dockerfile ./AnimalCenter
```
