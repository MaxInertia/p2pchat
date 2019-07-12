lazy:
	@gcc src/*.c -o bin/main
	@./bin/main 198.169.133.161 25

clean:
	@rm -rf bin
	@mkdir bin
	@echo "cleaned bin"