#!/bin/bash

flatc --rust --gen-onefile -o ../rust/dchat/src/idl base.fbs partner.fbs
