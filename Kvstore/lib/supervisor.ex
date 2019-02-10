defmodule Kvstore.Supervisor do
  use Supervisor

  def start_link(opts) do
    Supervisor.start_link(__MODULE__, :ok, opts)
  end

  @impl true
  def init(:ok) do
    children = [
      {DynamicSupervisor, name: Kvstore.BucketSupervisor, strategy: :one_for_one},
      {Kvstore.Registry,  name: Kvstore.Registry},
    ]
    Supervisor.init(children, strategy: :one_for_all)
  end
end
