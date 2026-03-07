using Profile;
using StaticArrays
using Base.Iterators: drop, reverse, take


example = """
987654321111111
811111111111119
234234234234278
818181911112111"""

function get_max_claude_with_vector!(line::String, digits_arr::Vector{UInt32})
    # Popola l'array preallocato
    for (i, c) in enumerate(line)
        digits_arr[i] = UInt32(c - '0')
    end

    n = length(line)
    max_val = UInt32(0)

    for i in 1:n-1
        base = digits_arr[i] * 10
        for j in i+1:n
            pair = base + digits_arr[j]
            if pair > max_val
                max_val = pair
            end
        end
    end

    return max_val
end
const LEN = 100
function get_max_claude_with_static_array!(line::String, digits_arr::MVector{LEN,UInt32})
    # Popola l'array preallocato
    max_val = UInt32(0)

    for i in 1:LEN
        digits_arr[i] = UInt32(line[i] - '0')
    end


    for i in 1:LEN-1
        base = digits_arr[i] * UInt32(10)
        for j in i+1:LEN
            pair = base + digits_arr[j]
            if pair > max_val
                max_val = pair
            end
        end
    end

    return max_val
end

function get_max_claude_arr(line)::Int
    max_val::Int = 0

    # Converti i caratteri in cifre numeriche una sola volta
    digits = [Int(ch) - 48 for ch in line]
    n = length(digits)

    for i in 1:n-1
        d1 = digits[i]
        # Calcola il contributo della prima cifra (moltiplicata per 10)
        base = d1 * 10

        for j in i+1:n
            # Somma la seconda cifra
            pair = base + digits[j]
            if pair > max_val
                max_val = pair
            end
        end
    end

    return max_val
end

# acc = []
function get_max_indicesSStar(line::String)::Int
    # global acc
    max::Int = 0
    len = length(line)

    for n1 in 1:len
        for n2 in n1+1:len
            ch1 = line[n1]
            ch2 = line[n2]
            pair = parse(Int, ch1 * ch2)

            # t1 = time()
            # t2 = time()
            # push!(acc, delta)

            if pair > max
                max = pair
            end
        end
    end
    return max
end

function get_max_indicesSDollar(line::String)::Int
    max::Int = 0
    len = length(line)
    for n1 in 1:len
        for n2 in n1+1:len
            ch1 = line[n1]
            ch2 = line[n2]
            pair = parse(Int, "$ch1$ch2")
            #pair = parse(Int, ch1 * ch2)
            if pair > max
                max = pair
            end
        end
    end
    return max
end

function get_max_opt(line)::Int
    max::Int = 0

    len = length(line)
    line_gen = [(Int(i) - 48) for i in line]

    for n1 in 1:len-1
        ch1 = line_gen[n1] * 10
        for n2 in n1+1:len
            pair = ch1 + line_gen[n2]
            if pair > max
                max = pair
            end
        end
    end
    return max
end

function get_max_digits(line)::Int
    max::Int = 0

    len = length(line)

    for n1 in (1:len-1)
        ch1 = (Int(line[n1]) - 48) * 10
        for n2 in n1+1:len
            pair = ch1 + Int(line[n2]) - 48
            if pair > max
                max = pair
            end
        end
    end
    return max
end


function get_max(line)::Int
    max::Int = 0
    for (n1, ch1) in enumerate(line[1:end])
        for ch2 in line[n1+1:end]
            pair = parse(Int, ch1 * ch2)
            if pair > max
                max = pair
            end
        end
    end
    return max
end
function get_max_claude(line)::Int
    max_val::Int = 0

    # Converti i caratteri in cifre numeriche una sola volta
    digits = [Int(ch) - 48 for ch in line]
    n = length(digits)

    for i in 1:n-1
        d1 = digits[i]
        # Calcola il contributo della prima cifra (moltiplicata per 10)
        base = d1 * 10

        for j in i+1:n
            # Somma la seconda cifra
            pair = base + digits[j]
            if pair > max_val
                max_val = pair
            end
        end
    end

    return max_val
end


@views function get_max_linear(line::String)::Int
    ints_char::Vector{Int} = [Int(i) - Int('0') for i in line]
    decina = 0
    pos_decina = 0
    unit = 0

    @inbounds for (n, ch) in enumerate(ints_char[1:end-1])
        if ch > decina
            decina = ch
            pos_decina = n
        end
    end

    @inbounds for ch in ints_char[pos_decina+1:end]
        if ch > unit
            unit = ch
        end
    end

    decina * 10 + unit
end

@views function get_max_linear_v2(line::String)::Int
    ints_char::Vector{Int} = [Int(i) - Int('0') for i in line]
    pos_decina = argmax(ints_char[1:end-1])
    ints_char[pos_decina] * 10 + maximum(ints_char[pos_decina+1:end])
end

@views function get_max_linear_no_alloc(line::String, digits_arr::MVector{UInt32})::UInt32
    for (i, ch) in enumerate(line)
        digits_arr[i] = UInt32(ch - '0')
    end

    pos_decina = argmax(digits_arr[1:end-1])

    unit = maximum(digits_arr[pos_decina+1:end])
    decina = digits_arr[pos_decina]

    decina * UInt32(10) + unit
end
@views function get_max_linear_no_alloc_Int(line::String, digits_arr::MVector{100,UInt8})::Int
    for (i, ch) in enumerate(line)
        digits_arr[i] = Int(ch - '0')
    end

    pos_decina = argmax(digits_arr[1:end-1])

    unit = maximum(digits_arr[pos_decina+1:end])
    decina = digits_arr[pos_decina]

    decina * 10 + unit
end

# @profview main()

lines = readlines("./src/input.txt")
digits_arr = MVector{100,UInt32}(undef)
function main_v2(lines)::Int
    #t1 = time_ns()
    #digits_arr = Vector{UInt32}(undef, 100)  # preallocato
    #digits_arr = MVector{100,UInt32}(undef)
    #solution = sum(get_max_claude_with_vector!(line, digits_arr) for line in lines)

    solution = 0
    for line in lines
        solution += get_max_linear_v2(line)
    end


    #solution = sum(get_max_claude_arr.(lines))

    #t2 = time_ns()
    #delta = (t2 - t1) / (1000.0 * 1000.0)
    #println("$solution duration: $delta ms")
    solution
end
function main_v1(lines)::Int

    solution = 0
    for line in lines
        solution += get_max_linear(line)
    end

    solution
end

function main_v3(lines)::UInt32

    solution = UInt32(0)
    #digits_arr = [0 for _i in 1:100]
    digits_arr = MVector{100,UInt32}(0 for _i in 1:100)
    for line in lines
        solution += get_max_linear_no_alloc(line, digits_arr)
    end

    solution
end

function get_max_part2(line, ::Val{N}, stack::MVector{N,Int})where N

    count = 0
    remaining = length(line)

    for b in codeunits(line)
        while count > 0 && stack[count] < b && (N - count) < remaining
            count -= 1
        end

        if count < N
            count += 1
            stack[count] = b
        end
        remaining -= 1
    end

    result = 0
    for i in 1:N
        result = result * 10 + (stack[i] - 48)
    end
    return result
end

const N = 2
function main_v5(lines)::Int
    stack = MVector{N,Int}(0 for _i in 1:N)
    sum(lines) do line
        get_max_part2(line, Val(N), stack)
    end

end

using Base.Threads
using StaticArrays


function main_v5_par(lines::Vector{String})::Int

    total = [0 for _i in 1:nthreads()]

    @threads for line in lines
        tid = threadid()
        stack = MVector{N,Int}(0 for _i in 1:N)

        @inbounds total[tid-1] += get_max_part2(line, Val(N), stack)
    end

    total |> sum
end

function main_v4(lines)::Int
    #t1 = time_ns()
    #digits_arr = [0 for _i in 1:100]
    solution = 0
    digits_arr = MVector{100,UInt8}(undef)
    for line in lines
        solution += get_max_linear_no_alloc_Int(line, digits_arr)
    end
    #t2 = time_ns()
    #delta = (t2 - t1) / (1000.0 * 1000.0)
    #println("$solution duration: $delta ms")
    solution
end
main_v5(lines)


#main(lines)
#main(lines)