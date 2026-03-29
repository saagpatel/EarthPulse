.PHONY: install build lint test dev clean

install:
	pnpm install

build:
	pnpm run build

lint:
	pnpm run lint

test:
	pnpm run test:all

dev:
	pnpm run dev

clean:
	pnpm run clean
