all: mc

test: mc-tests
	./mc-tests

docs: doc/regexp/index.html

doc/%/index.html: %.rs
	rustdoc $<

LIB_FILES := src/lexer.rs \
             src/parser.rs \
             src/span.rs \
             src/ast/mod.rs \
             src/ast/visit.rs \
             src/ast/defmap.rs \

MC_FILES := src/main.rs \
            $(LIB_FILES)

MC2C_FILES := src/mc2c.rs \
              $(LIB_FILES)

mc: $(MC_FILES)
	rustc $< -o $@

mc2c: $(MC2C_FILES)
	rustc $< -o $@

mc-tests: $(MC_FILES)
	rustc --test $< -o $@

.PHONY: all test docs clean
clean:
	rm -rf *~ doc mc mc-tests
