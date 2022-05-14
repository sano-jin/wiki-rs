#!/bin/bash

rm -rf docs/public
cp -r public docs/
rm docs/public/db/users/*
