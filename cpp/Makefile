NAME = filecrawler

CXX = clang++

CXXFLAGS = -Wall -Wextra -Werror -std=c++17

all: $(NAME)

$(NAME): main.o
	clang++ -Wall -Wextra -Werror main.o -o $(NAME) -lcrypto -pthread

clean:
	rm -f main.o

fclean: clean
	rm -f $(NAME)

re: fclean all