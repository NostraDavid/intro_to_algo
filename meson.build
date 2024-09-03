project('algorithms', 'c')

# Define the list of executable names (without extension)
executables = [
  'insertion_sort',
]

# Create an executable for each .c file
foreach executable : executables
    src_file = 'src/' + executable + '.c'  # Construct the source file path
    executable(executable, src_file)
endforeach
