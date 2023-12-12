build:
	@echo "Building.."
	@go build -o ./build/chlorine ./cmd/chlorine

run:
	@go run ./cmd/chlorine

dev:
	@air .

.PHONY: build run