.PHONY: all clean fclean re

NAME = Matt_daemon

CC = g++ -std=c++20
CFLAGS += -Wall -Wextra -Werror -pedantic -g 

# **************************************************************************** #
#                                  SOURCES COMMON                              #
# **************************************************************************** #

OBJ_DIR = ./obj/
SRC_DIR = ./src/
SRC_ = Server.cpp \
				file_handler.cpp \
				MailConfig.cpp \
				main.cpp \
				signal.cpp \
				TintinReporter.cpp \

O_FILES = $(SRC_:%.cpp=$(OBJ_DIR)%.o)

SRC = $(addprefix $(SRC_COMMON_DIR), $(SRC_COMMON_))


# **************************************************************************** #
#                                     INCLUDES                                 #
# **************************************************************************** #

INC_DIR = ./includes/

INCLUDES_ = Server.class.hpp \
				TintinReporter.class.hpp \
				common.hpp

INCLUDES = $(addprefix $(INC_DIR), $(INCLUDES_))
INCLUDE = -I $(INC_DIR) -lcurl


# **************************************************************************** #
#                                    RULES                                     #
# **************************************************************************** #

all: $(NAME)

$(NAME): $(OBJ_DIR) $(O_FILES) 
	$(info $(O_FILES))
	@printf "\r\033[K[$(NAME)] \033[1;32mLinking...\033[0m"
	$(CC) $(CFLAGS) -o $(NAME) $(O_FILES) $(INCLUDE)
	@printf "\r\033[K[$(NAME)] \033[1;32mDone!\033[0m\n"


$(OBJ_DIR):
	mkdir -p $@

# $(OBJ_DIR)%.o: $(SRC_DIR)%.cpp $(INCLUDES) // Doesnt work ????
$(OBJ_DIR)%.o: $(SRC_DIR)%.cpp ./includes/Server.class.hpp ./includes/TintinReporter.class.hpp ./includes/main.hpp
	$(CC) $(CFLAGS) -o $@ -c $< $(INCLUDE)

clean:
	@printf "[$(NAME)]  \033[1;31mCleaned .o!\033[0m\n"
	@rm -rf $(OBJ_DIR)

fclean: clean
	@printf "[$(NAME)]  \033[1;31mCleaned .a!\033[0m\n"
	@rm -f $(NAME)

re: fclean all
