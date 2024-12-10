using CairoMakie

include("../../../../bench_results.jl")

begin
	categories = Vector{Int64}()
	my_values = Vector{Float64}()
	dodge = Vector{Int64}()
	algo2categories = Dict{String, Int64}()
	mesh2categories = Dict{String, Int64}()
	for data_set in data
		for value in data_set[2]
			sp = split(data_set[1], "_")
			algo = sp[3]
			mesh = sp[1] * " " * sp[2]
			if !haskey(algo2categories, algo)
				algo2categories[algo] = length(algo2categories) + 1
			end
			if !haskey(mesh2categories, mesh)
				mesh2categories[mesh] = length(mesh2categories) + 1
			end
			push!(categories, mesh2categories[mesh])
			push!(my_values, 1.0 / value)
			push!(dodge, algo2categories[algo])
		end
	end

	# make sure that empty measurement combinations have one entry
	#=for algo in keys(algo2categories)
		for mesh in keys(mesh2categories)
			found = false
			for data_set in data
				sp = split(data_set[1], "_")
				if sp[3] == algo && sp[2] * "_" * sp[1] == mesh
					@show sp
					found = true
					break
				end
			end
			if !found
				println("Adding empty entry for $algo and $mesh")
				push!(categories, mesh2categories[mesh])
				push!(my_values, 0.0)
				push!(dodge, algo2categories[algo])
			end
		end
	end=#

	colors = Makie.wong_colors()

	# make a boxplot using Makie and save it as svg
	fig = Figure(resolution = (1200, 600))
	ax = Axis(fig[1, 1], yscale=log2, ylabel = "FPS",
		xticks = (collect(values(mesh2categories)), collect(keys(mesh2categories))))
	boxplot!(ax, categories, my_values, dodge = dodge, show_notch = true, color = colors[dodge])

	Legend(fig[1, 2], [PolyElement(polycolor = colors[i]) for i in 1:length(keys(algo2categories))], collect(keys(algo2categories)), "Algorithms")

	save("./boxplot.png", fig)
	save("./assets/fps_boxplot.svg", fig)
end

