#ifndef MAIN_HPP
#define MAIN_HPP

/* #define NSIG 32; */
extern const char *LOCK_FILE;
extern const char *LOG_FILE;

void close_fds();
void unlock_file();
void unlock_file_and_exit(int status);
long get_max_fd();
void create_lock_file(const char *filename);
void redirect_stream();
void set_sig_handlers();

#endif
