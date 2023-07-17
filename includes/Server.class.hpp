#ifndef CLIENT_CLASS_H
#define CLIENT_CLASS_H

#include <string>

enum ShellMode { None, Shell, Bash };

inline std::ostream &operator<<(std::ostream &os, ShellMode &mode);

class Server {
  public:
    Server(void);
    Server(Server const &reporter);
    Server &operator=(Server const &rhs);
    virtual ~Server(void);

    void set_mode(ShellMode mode);
    void *get_addr();
    void run();

  private:
    void *stream;
    void *addr;
    ShellMode mode;
};

#endif
