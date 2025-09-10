local dna_sequence = "TACAGATGACCGGATC"

local dna_to_rna = {
	["A"] = "U",
	["T"] = "A",
	["C"] = "G",
	["G"] = "C",
}

local function parsesequence(str)
	local output = ""
	for i = 1, string.len(str) do
		local letter = string.sub(str, i, i)
		letter = string.upper(letter)

		local to_rna = dna_to_rna[letter]

		if not to_rna then
			goto continue
		end

    output = output..to_rna

	    ::continue::
	end
  return output
end

print(parsesequence(dna_sequence))
