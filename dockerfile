FROM mongo:latest

RUN apt-get update && apt-get install -y mongodb-org-shell && apt-get clean

CMD ["bash"]
