webapp:
	npm ci --prefix=webapp
	npm run build --prefix=webapp
	rm -rf static
	mkdir static
	mv webapp/build/* static/

clean:
	rm -rf webapp/build static render/node_modules

.PHONY: webapp webapp/clean
