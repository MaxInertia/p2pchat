#ifndef _CONNECT_H
#define _CONNECT_H

enum connection_statuses
{
  FAIL,
  SUCCESS,
  PENDING,
};

typedef int constat_t;

typedef struct
{
} con_config_t;

constat_t connect(const con_config_t *);

#endif //_CONNECT_H