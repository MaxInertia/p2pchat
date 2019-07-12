#include <stdio.h>
#include <stdlib.h>

#include "connect.h"

#define NUM_ARGS 3
#define MSG_BUF_SIZE 100

void send() {}

void recv() {}

const con_config_t *generate_config(const char *ip, const char *port)
{
  const con_config_t *empty_config = malloc(sizeof(con_config_t));
  return empty_config;
}

int send_and_receive(const con_config_t *config)
{
  printf("connecting...\n");
  connect(config);
  printf("success!\n");
  char *buf = (char *)malloc(MSG_BUF_SIZE * sizeof(char));
  while (1)
  {
    printf("You: ");
    scanf("%s", buf);
    printf("sending...\nsent!\n");
    // send it
    buf = (char *)realloc(buf, MSG_BUF_SIZE * sizeof(char));
  }
}

int main(int argc, char **argv)
{
  if (argc != NUM_ARGS)
  {
    printf("expected %d args but got %d\n", NUM_ARGS, argc);
    return 1;
  }
  const char *ip_address = argv[1];
  const char *port = argv[2];
  const con_config_t *config = generate_config(ip_address, port);
  const int status = send_and_receive(config);
  return status;
}