-- Add up migration script here
CREATE UNIQUE INDEX unq_addresses_address_number_zip_code ON addresses(address, number, zip_code);
