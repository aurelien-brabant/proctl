TARGET	:= ./target/release/proctl
RM		:= rm -rf
INSTALL_DIR	:= /usr/local/bin/proctl

all: $(TARGET)

$(TARGET):
	cargo build --release
.PHONY: $(TARGET)

install: all
	cp $(TARGET) /usr/local/bin
.PHONY: install

clean:
	$(RM) $(TARGET)
.PHONY: clean

fclean:
	$(RM) ./target
.PHONY: fclean

re: fclean all
.PHONY: re
