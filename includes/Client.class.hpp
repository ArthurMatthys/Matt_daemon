#ifndef CLIENT_CLASS_H
#define CLIENT_CLASS_H

#include <string>

enum ShellMode { None, Shell, Bash };

inline std::ostream &operator<<(std::ostream &os, ShellMode &mode);

class Client {
  public:
    Client(void);
    Client(Client const &reporter);
    Client &operator=(Client const &rhs);
    virtual ~Client(void);

    void set_mode(ShellMode mode);
    void *get_addr();

  private:
    void *stream;
    void *addr;
    ShellMode mode;
};

#endif
