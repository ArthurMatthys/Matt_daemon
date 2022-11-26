NAME = matt_daemon

# recursive wildcard function
# ($1): list of directories
# ($2) is a list of patterns to match.
# source: https://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# list of directories to search for source files
SRC := $(call rwildcard,./daemonize,*.rs *.toml)
SRC += $(call rwildcard,./usage,*.rs *.toml)

all: $(NAME) 

$(NAME): $(SRC)
	cargo build
	cp ./target/debug/matt_daemon .

clean:
	cargo clean

fclean: clean
	rm -rf matt_daemon

re: fclean all