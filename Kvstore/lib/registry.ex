defmodule Kvstore.Registry do
  require Logger
  use GenServer

  def start_link(opts) do
    GenServer.start_link(__MODULE__, :ok, opts)
  end

  def lookup(server, name) do
    GenServer.call(server, {:lookup, name})
  end

  def create(server, name) do
    GenServer.call(server, {:create, name})
  end

  def stop(server) do
    GenServer.stop(server)
  end

  @impl true
  def init(:ok) do
    names = %{}
    refs = %{}
    {:ok, {names, refs}}
  end

  @impl true
  def handle_call({:lookup, name}, _from, {names, _} = state) do
    {:reply, Map.fetch(names, name), state}
  end

  @impl true
  def handle_call({:create, name}, from, {names, refs} = state) do
    {pid, _} = from
    Logger.info "create #{name} caller #{pid}"
    state = 
      if !Map.has_key?(names, name) do
        {:ok, pid} = DynamicSupervisor.start_child(
          Kvstore.BucketSupervisor, Kvstore.Bucket)
        ref = Process.monitor(pid)
        refs = Map.put(refs, ref, name)
        names = Map.put(names, name, pid)
        {names, refs}
      else
        state
      end
    {:reply, {:ok, name}, state}
  end

  @impl true
  def handle_info({:DOWN, ref, :process, _pid, _reason}, {names, refs}) do
    {name, refs} = Map.pop(refs, ref)
    names = Map.delete(names, name)
    {:noreply, {names, refs}}
  end

  @impl true
  def handle_info(msg, state) do
    Logger.info "discard #{msg}"
    {:noreply, state}
  end
end
